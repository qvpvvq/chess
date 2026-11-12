import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';
import { ValidationPipe } from '@nestjs/common';

const PORT = process.env.PORT ?? 3005;

async function bootstrap() {
  const app = await NestFactory.create(AppModule);

  app.useGlobalPipes(
    new ValidationPipe({
      whitelist: true,
      forbidNonWhitelisted: true,
      forbidUnknownValues: true,
      transform: true,
    }),
  );

  app.enableCors({ origin: 'http://localhost:3000' });

  await app.listen(PORT, () => {
    console.log(
      `le application is currently running on http://localhost:${PORT}`,
    );
  });
}
bootstrap();
