import { User } from '../domain/user.model';

export class AddUserCommand {
  type = 'ADD_USER_COMMAND' as const;
  constructor(public payload: User) {}
}

export class ValidateUserCommand {
  type = 'VALIDATE_USER_COMMAND' as const;
  constructor(public payload: { eventId: string; userId: string }) {}
}

export type UserCommand = AddUserCommand | ValidateUserCommand;
