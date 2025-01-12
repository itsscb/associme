import { HttpEvent, HttpHandler, HttpInterceptor, HttpRequest } from '@angular/common/http';

export class JwtInterceptor implements HttpInterceptor {

  intercept(req: HttpRequest<any>, next: HttpHandler) {
    const token = localStorage.getItem('auth_token');
    if (token) {
      const tokenPayload = JSON.parse(token);

      if(tokenPayload.token.expires_at < Date.now()) {
        localStorage.removeItem('auth_token');
        return next.handle(req);
      }

      if(tokenPayload.token.token) {
        const authReq = req.clone({
          setHeaders: {
            Authorization: `Bearer ${tokenPayload.token.token}`
          }
        });
        return next.handle(authReq);
      } else {
        return next.handle(req);
      }
    } else {
      return next.handle(req);
    }
  }
}