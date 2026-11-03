import { Module } from '@nestjs/common';
import { PrismaModule } from './prisma/prisma.module';
import { ConfigModule } from '@nestjs/config';
import { CoreGameLogicModule } from './core-game-logic/core-game-logic.module';
@Module({
  imports: [
    PrismaModule,
    ConfigModule.forRoot({
      envFilePath: '.env',
      isGlobal: true,
    }),
    CoreGameLogicModule,
  ],
  controllers: [],
  providers: [],
})
export class AppModule {}
