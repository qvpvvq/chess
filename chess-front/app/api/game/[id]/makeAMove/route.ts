import { NextRequest, NextResponse } from 'next/server';

import axios from 'axios';

const backApi = process.env.BACK_API_URL;

export async function POST(_req: NextRequest, { params }: { params: Promise<{ id: string }> }) {
    const { id } = await params;

    const body = await _req.json();

    const { data } = await axios.post(`${backApi}/chess/${id}/move`, body);
    return NextResponse.json(data);
}
