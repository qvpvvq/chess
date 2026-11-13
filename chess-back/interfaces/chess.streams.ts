// поток для игры с движком (request - ходы игрока, response - ответные ходы движка)
const chessStreamSubjects = {
  requestAlias: 'chess.engine.request.>',
  responseAlias: 'chess.engine.response.>',
};

export const chessPublishSubjects = {
  request: 'chess.engine.request',
  response: 'chess.engine.response',
};

export const chessStream = {
  name: 'CHESS',
  subjects: [
    chessStreamSubjects.requestAlias,
    chessStreamSubjects.responseAlias,
  ],
  consumers: {
    // Читает RUST с StockFish
    engineWorker: {
      durable: 'engine_worker',
      filter: chessStreamSubjects.requestAlias,
    },
    // Читает Nest
    engineResponses: {
      durable: 'engine_response',
      filter: chessStreamSubjects.responseAlias,
    },
  },
};

export const chessPublishSubjectsBuilder = (gameid: string) => {
  return {
    request: `${chessPublishSubjects.request}.${gameid}`,
    response: `${chessPublishSubjects.response}.${gameid}`,
  };
};

export interface ChessRequestEvent {
  gameId: string;

  version: number;
  fen: string; // снапшот всей доски
  color: 'black' | 'white';
  history: string[]; // история ходов в UCI формате
}

export type ChessResponseEvent =
  | { gameId: string; version: number; uci: string; evalCp: number }
  | { gameId: string; version: number; error: string };

// Поток ии комментатора игры (request - запрос на комментирование response - комментарий ии)

const commentStreamSubjects = {
  requestAlias: 'chess.comment.request.>',
  responseAlias: 'chess.comment.response.>',
};

export const commentPublishSubjects = {
  request: 'chess.comment.request',
  response: 'chess.comment.response',
};

export const commentStream = {
  name: 'CHESS_COMMENTS',
  subjects: [
    commentStreamSubjects.requestAlias,
    commentStreamSubjects.responseAlias,
  ],
  consumers: {
    commentResponse: {
      durable: 'comment_response',
      filter: commentStreamSubjects.responseAlias, // консьюмит Nest
    },
    commentRequest: {
      durable: 'comment_request',
      filter: commentStreamSubjects.requestAlias, // консьюмит python llm
    },
  },
};

export const commentPublishSubjectsBuilder = (
  gameid: string,
  moveId?: string,
) => {
  return {
    request: moveId
      ? `${commentPublishSubjects.request}.${gameid}.${moveId}`
      : `${commentPublishSubjects.request}.${gameid}`,
    response: `${commentPublishSubjects.response}.${gameid}`,
  };
};

export interface CommentRequestEvent {
  type: 'INTRO' | 'MOVE';
  gameId: string;
  persona: string;
  previousComments: string[];

  // далее если тип комментария MOVE
  moveId?: string;
  ply?: number;
  author?: 'USER' | 'AI';
  color?: 'white' | 'black';
  san?: string;
  historySan?: string[];
  evalBeforeCp?: number | null;
  evalAfterCp?: number | null;
}

export interface CommentResponseEvent {
  gameId: string;
  moveId?: string;
  ply?: number;
  type: 'INTRO' | 'MOVE';
  text: string;
}
