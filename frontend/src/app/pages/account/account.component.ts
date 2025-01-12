import { Component, OnInit } from '@angular/core';
import { BackendService } from 'src/app/service/backend.service';
import { Account } from 'src/app/model/account.model';

@Component({
  selector: 'app-account',
  templateUrl: './account.component.html',
  styleUrls: ['./account.component.scss']
})
export class AccountComponent implements OnInit {

  constructor(private backend_service: BackendService) { }

  account?: Account;
  ngOnInit(): void {
    this.backend_service.get_account().subscribe({next: (account) => {
      this.account = account;
    }, error: (error) => {
      // TODO: Show error message to user
      console.error('Account error:', error);
    }});
  }

}
