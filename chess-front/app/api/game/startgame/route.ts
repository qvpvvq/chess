// app/api/start-game/route.ts
import { NextResponse } from 'next/server';
import axios from 'axios';

const backApi = process.env.BACK_API_URL;

export async function POST() {
    try {
        const { data } = await axios.post(`${backApi}/chess/start`);

        return NextResponse.json(data);
    } catch (error) {
        if (axios.isAxiosError(error)) {
            return NextResponse.json(
                { error: error.message, details: error.response?.data },
                { status: error.response?.status ?? 502 },
            );
        }
        return NextResponse.json({ error: 'Unknown error' }, { status: 500 });
    }
}
