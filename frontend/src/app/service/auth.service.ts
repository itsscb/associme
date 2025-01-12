import { HttpClient } from '@angular/common/http';
import { catchError, map, Observable, of } from 'rxjs';
import { Injectable } from '@angular/core';
import { environment } from 'src/environments/environment';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  constructor(private http: HttpClient) { 
    const token = localStorage.getItem('auth_token');
    if (token) {
      const tokenPayload = JSON.parse(token);
      const expiresAt = tokenPayload.token.expires_at;
      if (new Date(expiresAt).getTime() > Date.now()) {
        this.isLoggedIn = true;
      } else {
        this.isLoggedIn = false;
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

    return this.http.post<any>(environment.apiUrl + '/api/v1/login', body.toString(), {
      headers: { 'Content-Type': 'application/x-www-form-urlencoded' }
    })
      .pipe(
      map(response => {
            localStorage.setItem('auth_token', JSON.stringify(response));
          this.isLoggedIn = true;
          return true;
        }),
        catchError(error => {
          // TODO: Show error message to user
          this.isLoggedIn = false;
          return of(false);
        })
      );
  }

  logout(): void {
    localStorage.removeItem('auth_token');
    this.isLoggedIn = false;
  }

  isAuthenticated(): boolean {
    const token = localStorage.getItem('auth_token');
    if (token) {
      const tokenPayload = JSON.parse(token);
      const expiresAt = tokenPayload.token.expires_at;
      if (new Date(expiresAt).getTime() > Date.now()) {
        this.isLoggedIn = true;
        return true;
      } else {
        this.isLoggedIn = false;
      }
    }     
    
    return false;
  }
}