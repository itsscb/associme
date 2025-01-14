import { HttpClient } from "@angular/common/http";
import { inject, Injectable } from "@angular/core";
import { Observable } from "rxjs";
import { catchError } from "rxjs/operators";
import { Account } from "../model/account.model";
import { Router } from "@angular/router";
import { Member, NewMember } from "../model/member.model";

@Injectable({
  providedIn: "root",
})
export class BackendService {
  private apiUrl = "/api/v1";

  constructor(
    private http: HttpClient,
    private router: Router,
  ) {}

  create_member(member: NewMember): Observable<Member> {
    const json = JSON.stringify(member);
    console.log("create_member", json);
    return this.http
      .post<Member>(`${this.apiUrl}/member`, json, {
        headers: { "Content-Type": "application/json" },
      })
      .pipe(catchError(this.handleError));
  }

  get_account(): Observable<Account> {
    return this.http
      .get<Account>(`${this.apiUrl}/account`)
      .pipe(catchError(this.handleError));
  }

  private handleError(error: any): Observable<never> {
    // TODO: Show error message to user
    console.log("An error occurred:", error);
    // this.router.navigate(['/login']);
    throw error;
  }
}
