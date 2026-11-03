import { Color as ColorEnum } from 'generated/prisma';

export type Color = 'white' | 'black';
type PieceName = 'pawn' | 'knight' | 'bishop' | 'rook' | 'queen' | 'king';

export interface Figure {
  id: string;
  name: PieceName;
  color: Color;
  position: string | null;
}

export const NAMES: Record<string, PieceName> = {
  p: 'pawn',
  n: 'knight',
  b: 'bishop',
  r: 'rook',
  q: 'queen',
  k: 'king',
};

export const toColor = (c: 'w' | 'b'): ColorEnum =>
  c === 'w' ? ColorEnum.WHITE : ColorEnum.BLACK;

// типы для рокировки
// куда переезжает ладья при рокировке: [откуда, куда]
export const ROOK_CASTLE = {
  w: { king: ['h1', 'f1'], queen: ['a1', 'd1'] },
  b: { king: ['h8', 'f8'], queen: ['a8', 'd8'] },
};
