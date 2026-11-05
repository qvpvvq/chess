import { QueryClient } from '@tanstack/react-query';

import axios from 'axios';

export default function appMutations(queryClient: QueryClient) {
    queryClient.setMutationDefaults(['makeAMove'], {
        mutationFn: async ({ id, ...body }) => {
            const { data } = await axios.post(`/api/game/${id}/makeAMove`, body);
            return data;
        },
        onError: (err) => console.error('ход не прошёл', err),
    });
    queryClient.setMutationDefaults(['startgame'], {
        mutationFn: async () => {
            const { data } = await axios.post('/api/game/startgame');
            return data;
        },
    });
}
