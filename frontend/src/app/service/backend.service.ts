import { HttpClient } from "@angular/common/http";
import { inject, Injectable } from "@angular/core";
import { Observable } from "rxjs";
import { catchError, map } from "rxjs/operators";
import { Account } from "../model/account.model";
import { Router } from "@angular/router";
import { Member, Membership, NewMember, UpdateMember } from "../model/member.model";

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
    // console.log("create_member", json);
    return this.http
      .post<Member>(`${this.apiUrl}/member`, json, {
        headers: { "Content-Type": "application/json" },
      })
      .pipe(catchError(this.handleError));
  }

  update_member(id: string, member: UpdateMember): Observable<Member> {
    const json = JSON.stringify(member);
    return this.http
      .patch<{member: Member}>(`${this.apiUrl}/member/${id.trim()}`, json, {
        headers: { "Content-Type": "application/json" },
      })
      .pipe(map(response => {
        
        const member = response.member as Member;
        const transformedMember: Member = {
        ...member,
        birthday: new Date(member.birthday),
        created_at: new Date(member.created_at),
        changed_at: new Date(member.changed_at),
        membership_state: member.membership_state as Membership,
        };
        return transformedMember;
      }), catchError(this.handleError));
  }

  list_members(): Observable<Member[]> {
    return this.http
      .get<{members: Member[]}>(`${this.apiUrl}/member`)
      .pipe(
        map(response => response.members.map(member => ({
          ...member,
          birthday: new Date(member.birthday),
          created_at: new Date(member.created_at),
          changed_at: new Date(member.changed_at),
          membership_state: member.membership_state as Membership,
        }))),
        catchError(this.handleError)
      );
  }

  delete_member(id: string): Observable<void> {
    return this.http
      .delete<void>(`${this.apiUrl}/member/${id.trim()}`)
      .pipe(catchError(this.handleError));
  }

  get_member(id: string): Observable<Member> {
    return this.http
      .get<{member: Member}>(`${this.apiUrl}/member/${id.trim()}`)
      .pipe(
      map(response => {
        const member = response.member;
        const transformedMember = {
        ...member,
        birthday: new Date(member.birthday),
        created_at: new Date(member.created_at),
        changed_at: new Date(member.changed_at),
        membership_state: member.membership_state as Membership,
        };
        return transformedMember;
      }),
      catchError(this.handleError)
      );
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
