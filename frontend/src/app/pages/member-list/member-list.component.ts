import { Component, ElementRef, OnInit, ViewChild } from '@angular/core';
import { Member, Membership } from 'src/app/model/member.model';
import { BackendService } from 'src/app/service/backend.service';

import { Table } from 'primeng/table';


@Component({
  selector: 'app-member-list',
  templateUrl: './member-list.component.html',
  styleUrls: ['./member-list.component.scss']
})
export class MemberListComponent implements OnInit{
  constructor(private backend_service: BackendService) {}

  ngOnInit() {
    this.backend_service.list_members().subscribe((members) => {
      this.members = members.map(member => ({
        ...member,
        birthday: new Date(member.birthday),
        created_at: new Date(member.created_at),
        changed_at: new Date(member.changed_at),
        membership_state: member.membership_state as Membership,
      }));
    });
  }
  
      @ViewChild('table') table: Table;
  
      @ViewChild('filter') filter: ElementRef;

  members: Member[] = [
    {
      id:"z",
      email:"a@a.de",
      phone:"1",
      first_name:"a_f",
      last_name:"z_l",
      birthday: new Date("2007-01-17T17:13:17.143Z"),
      postalcode:"1",
      city:"a_c",
      street:"a_s",
      house_number:"1",
      membership_state: Membership.Active,
      created_at: new Date("2025-01-17T17:13:17.143Z"),
      created_by:"",
      changed_at: new Date("2025-01-17T17:13:17.143Z"),
      changed_by:"",
      member_id:"99"
    },
    {
      id:"b2",
      email:"b@b.de",
      phone:"2",
      first_name:"b_f",
      last_name:"b_l",
      birthday: new Date("2001-02-17T17:13:17.143Z"),
      postalcode:"2",
      city:"b_c",
      street:"b_s",
      house_number:"2",
      membership_state: Membership.Active,
      created_at: new Date("2025-01-17T17:13:17.143Z"),
      created_by:"system",
      changed_at: new Date("2025-01-17T17:13:17.143Z"),
      changed_by:"system",
      member_id:"2"
    },
    {
      id:"b2",
      email:"b@b.de",
      phone:"2",
      first_name:"b_f",
      last_name:"b_l",
      birthday: new Date("2001-02-17T17:13:17.143Z"),
      postalcode:"2",
      city:"b_c",
      street:"b_s",
      house_number:"2",
      membership_state: Membership.Passive,
      created_at: new Date("2025-01-17T17:13:17.143Z"),
      created_by:"system",
      changed_at: new Date("2025-01-17T17:13:17.143Z"),
      changed_by:"system",
      member_id:"3"
    },
  ];

   clear_filter(table: Table) {
     table.sortField = 'last_name';
          table.clear();

          this.filter.nativeElement.value = '';
      }
}
