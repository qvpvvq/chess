import { GameBoard } from '@/app/components/GameBoard';
import { Suspense } from 'react';

async function GameBoardWithId({ params }: { params: Promise<{ id: string }> }) {
    const { id } = await params;
    return <GameBoard sessionId={id} />;
}

export default function Session({ params }: { params: Promise<{ id: string }> }) {
    return (
        <main className="mx-auto flex max-w-5xl flex-col gap-6 p-6">
            <h1 className="text-2xl font-semibold">Партия</h1>
            <Suspense fallback={''}>
                <GameBoardWithId params={params} />
            </Suspense>
        </main>
    );
}
