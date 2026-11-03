import { IsIn, IsOptional, IsString } from 'class-validator';

export class MoveDto {
  @IsString()
  from: string;

  @IsString()
  to: string;

  @IsOptional()
  @IsIn(['q', 'r', 'b', 'n'])
  promotion?: 'q' | 'r' | 'b' | 'n';
}
