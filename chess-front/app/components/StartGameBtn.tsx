'use client';

import { useRouter } from 'next/navigation';
import { useMutation } from '@tanstack/react-query';

export function StartGameButton() {
    const router = useRouter();

    const { mutate, isPending, isError, error } = useMutation({
        mutationKey: ['startgame'],
        onSuccess: (game) => router.push(`/session/${game.id}`),
    });

    return (
        <div className="flex flex-col items-center gap-3">
            <button
                onClick={() => mutate()}
                disabled={isPending}
                className="rounded-xl bg-neutral-900 px-8 py-3 text-lg font-medium text-white transition hover:bg-neutral-700 disabled:opacity-60">
                {isPending ? 'Создаём партию…' : 'Начать игру'}
            </button>

            {isError && <p className="text-sm text-red-600">{(error as Error).message}</p>}
        </div>
    );
}
