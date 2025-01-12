import { Component, inject } from '@angular/core';
import { AuthService } from '../service/auth.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-login',
  templateUrl: './app.login.component.html',
})
export class AppLoginComponent {
  constructor(private auth_service: AuthService) { }
  router = inject(Router);

  dark: boolean;

  checked: boolean;
  email: string;
  password: string;

  login() {
    this.auth_service.login({ email: this.email, password: this.password }).subscribe(
      (response) => {
        if (response) {
          this.router.navigate(['/account']);
        } 
      }
    );
  }
}
