'use client';

import { useMemo, useState } from 'react';
import Link from 'next/link';
import { Chess, Square } from 'chess.js';
import { Chessboard } from 'react-chessboard';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import axios from 'axios';

export function GameBoard({ sessionId }: { sessionId: string }) {
    const queryClient = useQueryClient();
    const key = ['game', sessionId];

    const {
        data: game,
        isLoading,
        isError,
    } = useQuery({
        queryKey: key,
        queryFn: async () => {
            const { data } = await axios.get(`/api/game/${sessionId}`);
            return data;
        },
    });

    const { mutate, isPending, error } = useMutation({
        mutationKey: ['makeAMove'],

        // оптимистично двигаем фигуру в кэше
        onMutate: async ({ from, to }) => {
            await queryClient.cancelQueries({ queryKey: key });
            const prev = queryClient.getQueryData(key);
            if (prev) {
                try {
                    const chess = new Chess(prev.fen);
                    chess.move({ from, to, promotion: 'q' });
                    queryClient.setQueryData(key, { ...prev, fen: chess.fen() });
                } catch {}
            }
            return { prev };
        },

        // сервер отклонил: возвращаем то, что было
        onError: (_e, _v, ctx) => ctx?.prev && queryClient.setQueryData(key, ctx.prev),

        // в любом случае сверяемся с сервером
        onSettled: () => queryClient.invalidateQueries({ queryKey: key }),
    });

    const [selected, setSelected] = useState<Square | null>(null);

    const fen = game?.fen;
    const chess = useMemo(() => (fen ? new Chess(fen) : null), [fen]);

    const legalTargets = useMemo(
        () =>
            chess && selected
                ? chess.moves({ square: selected, verbose: true }).map((m) => m.to)
                : [],
        [chess, selected],
    );

    if (isLoading) return <p className="p-6">Загрузка партии…</p>;
    if (isError || !game || !chess)
        return <p className="p-6 text-red-600">Не удалось загрузить партию</p>;

    function tryMove(from: Square, to: Square): boolean {
        if (isPending || game!.status !== 'active') return false;
        try {
            new Chess(game!.fen).move({ from, to, promotion: 'q' }); // быстрая локальная проверка
        } catch {
            return false;
        }
        setSelected(null);
        mutate({ id: sessionId, from, to });
        return true;
    }

    const squareStyles: Record<string, React.CSSProperties> = {};
    if (selected) squareStyles[selected] = { background: 'rgba(255, 215, 0, 0.45)' };
    for (const sq of legalTargets) {
        squareStyles[sq] = {
            background: 'radial-gradient(circle, rgba(0,0,0,.28) 24%, transparent 26%)',
        };
    }

    const moves = (game.moves ?? []).map((m) => m.san);

    const status = chess.isCheckmate()
        ? `Мат. Победили ${chess.turn() === 'w' ? 'чёрные' : 'белые'}`
        : chess.isStalemate()
        ? 'Пат'
        : chess.isDraw()
        ? 'Ничья'
        : `Ход ${chess.turn() === 'w' ? 'белых' : 'чёрных'}${chess.inCheck() ? ' (шах)' : ''}`;

    const errorText = error
        ? [error.response?.data?.message ?? 'Ошибка запроса'].flat().join(', ')
        : null;

    return (
        <div className="flex flex-col gap-4">
            <h1 className="text-sm text-neutral-400">Партия: {sessionId}</h1>
            <div className="flex flex-col gap-6 md:flex-row">
                <div className="w-full max-w-[560px]">
                    <Chessboard
                        options={{
                            position: game.fen,
                            squareStyles,
                            onPieceDrop: ({ sourceSquare, targetSquare }) =>
                                targetSquare
                                    ? tryMove(sourceSquare as Square, targetSquare as Square)
                                    : false,
                            onSquareClick: ({ square }) => {
                                const sq = square as Square;
                                if (selected && legalTargets.includes(sq)) tryMove(selected, sq);
                                else setSelected(sq);
                            },
                        }}
                    />
                </div>

                <aside className="flex w-full flex-col gap-4 md:w-64">
                    <div className="rounded-lg border p-3 font-medium">{status}</div>

                    {errorText && (
                        <div className="rounded-lg border border-red-300 p-3 text-sm text-red-600">
                            {errorText}
                        </div>
                    )}

                    <div className="max-h-96 overflow-y-auto rounded-lg border p-3">
                        <h2 className="mb-2 text-sm font-semibold text-neutral-500">Ходы</h2>
                        {moves.length === 0 ? (
                            <p className="text-sm text-neutral-400">Пока нет ходов</p>
                        ) : (
                            <ol className="grid grid-cols-[2rem_1fr_1fr] gap-x-2 text-sm">
                                {Array.from({ length: Math.ceil(moves.length / 2) }, (_, i) => (
                                    <li key={i} className="contents">
                                        <span className="text-neutral-400">{i + 1}.</span>
                                        <span>{moves[i * 2]}</span>
                                        <span>{moves[i * 2 + 1] ?? ''}</span>
                                    </li>
                                ))}
                            </ol>
                        )}
                    </div>
                </aside>
            </div>
        </div>
    );
}
