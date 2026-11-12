import { useQueryClient } from '@tanstack/react-query';
import { useEffect, useState } from 'react';

export function useChessMoveEvents(sessionId: string) {
    const queryClient = useQueryClient();
    const [aiError, setAiError] = useState<boolean>(false);

    useEffect(() => {
        const es = new EventSource(
            `${process.env.NEXT_PUBLIC_BACK_API_URL}/chess/game/${sessionId}/events`,
        );

        const refresh = () => queryClient.invalidateQueries({ queryKey: ['game', sessionId] });

        es.addEventListener('move', () => {
            setAiError(false);
            refresh();
        });

        es.addEventListener('ai_error', () => {
            setAiError(true);
            refresh();
        });
    }, [sessionId, queryClient]);

    return { aiError };
}
