import { HttpClient } from '@angular/common/http';
import { catchError, map, Observable, of } from 'rxjs';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  constructor(private http: HttpClient) { 
    const token = localStorage.getItem('JWT_Token');
    console.log('token', token, 'NOW', Date.now());
    if (token) {
      const tokenPayload = JSON.parse(token);
      const expiresAt = tokenPayload.expires_at;
      if (new Date(expiresAt).getTime() > Date.now()) {
        this.isLoggedIn = true;
      } else {
        this.isLoggedIn = false;
        localStorage.removeItem('JWT_Token');
      }
    } else {
      this.isLoggedIn = false;
    }
  }

  isLoggedIn: boolean = false;

  login(userDetails: { email: string; password: string }): Observable<boolean> {
    const body = new URLSearchParams();
    body.set('email', userDetails.email);
    body.set('password', userDetails.password);

    return this.http.post<any>('http://127.0.0.1:8000/api/account/login', body.toString(), {
      headers: { 'Content-Type': 'application/x-www-form-urlencoded' }
    })
      .pipe(
      map(response => {
            localStorage.setItem('JWT_Token', JSON.stringify(response));
          this.isLoggedIn = true;
          return true;
        }),
        catchError(error => {
          console.log(error);
          this.isLoggedIn = false;
          return of(false);
        })
      );
  }

  logout(): void {
    localStorage.removeItem('JWT_Token');
    this.isLoggedIn = false;
  }

  isAuthenticated(): boolean {
    return this.isLoggedIn;
  }
}