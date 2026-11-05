import { StartGameButton } from '@/app/components/StartGameBtn';

export default function StartGame() {
    return (
        <main className="flex min-h-screen flex-col items-center justify-center gap-8 p-6 text-center">
            <div className="flex flex-col gap-3">
                <h1 className="text-4xl font-bold tracking-tight">AI Chess</h1>
                <p className="max-w-md text-neutral-500">
                    Играй в шахматы и получай подсказки и разбор ходов от ИИ.
                </p>
            </div>

            <StartGameButton />
        </main>
    );
}
