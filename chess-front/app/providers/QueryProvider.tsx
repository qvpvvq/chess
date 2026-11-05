'use client';

import { ReactNode, useState } from 'react';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import appMutations from '@/lib/mutations';

export function QueryProvider({ children }: { children: ReactNode }) {
    const [queryClient] = useState(() => {
        const client = new QueryClient();
        appMutations(client);
        return client;
    });

    return <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>;
}
