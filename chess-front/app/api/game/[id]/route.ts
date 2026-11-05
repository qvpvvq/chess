import { NextRequest, NextResponse } from 'next/server';
import axios from 'axios';

const backApi = process.env.BACKEND_URL ?? 'http://localhost:3005';

export async function GET(_req: NextRequest, { params }: { params: Promise<{ id: string }> }) {
    const { id } = await params;

    const { data } = await axios.get(`${backApi}/chess/game/${id}`);
    return NextResponse.json(data);
}
