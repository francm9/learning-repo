import { Controller, Post, Req } from '@nestjs/common';
import { AppService } from './app.service';

@Controller()
export class AppController {
  constructor(private readonly appService: AppService) {}

  @Post('hello')
  getHello(@Req() request: Request): string {
    console.log('Request:', request.body);
    return this.appService.getHello();
  }
}
