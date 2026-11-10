const chessStreamSubjects = {
  requestAlias: 'chess.ai.request.>',
  responseAlias: 'chess.ai.response.>',
};

export const chessPublishSubjects = {
  request: 'chess.ai.request',
  response: 'chess.ai.response',
};

export const chessStream = {
  name: 'CHESS',
  subjects: [
    chessStreamSubjects.requestAlias,
    chessStreamSubjects.responseAlias,
  ],
  consumers: {
    // Читает Python
    aiWorker: {
      durable: 'ai_worker',
      filter: chessStreamSubjects.requestAlias,
    },
    // Читает Nest
    aiResponses: {
      durable: 'ai_response',
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
  | { gameId: string; version: number; uci: string }
  | { gameId: string; version: number; error: string };
