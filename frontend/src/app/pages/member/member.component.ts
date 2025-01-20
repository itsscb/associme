import { Component, OnInit } from "@angular/core";
import { CommonModule } from "@angular/common";

import {
  Member,
  member_to_new,
  Membership,
  new_member,
  NewMember,
  UpdateMember,
  validate_member,
} from "src/app/model/member.model";
import { BackendService } from "src/app/service/backend.service";
import { BreadcrumbService } from "src/app/breadcrumb.service";
import { ActivatedRoute } from "@angular/router";

@Component({
  selector: "app-member",
  templateUrl: "./member.component.html",
  // styleUrls: ["./member.component.scss"],
})
export class MemberComponent implements OnInit {
  constructor(
    private backend_service: BackendService,
    private breadcrumb_service: BreadcrumbService,
    private route: ActivatedRoute,
  ) {
    this.breadcrumb_service.setItems([
      { label: "Member", routerLink: ["/member"] },
      { label: "New", },
    ]);
  }

  ngOnInit() {
    this.user_id = this.route.snapshot.paramMap.get("id");
    if (this.user_id === 'new') {
      this.breadcrumb_service.setItems([
        { label: "Member", routerLink: ["/member"] },
        { label: "New", },
      ]);
      const member = new_member();
      this.set_member(member);
    }
    else if(this.user_id) {
      this.backend_service.get_member(this.user_id).subscribe((member) => {
        // console.table(member);
        this.set_member(member);
        this.breadcrumb_service.setItems([
          { label: "Member", routerLink: ["/member"] },
          { label: `${member.last_name}, ${member.first_name}${member.member_id ? ` [${member.member_id}]` : ''}`, },
        ]);
      });
      
    }
    this.max_year = new Date();
    this.max_year.setFullYear(this.max_year.getFullYear() - 18);
  }

  user_id: string;

  max_year: Date;


  membership_options = [
    Membership.None,
    Membership.Active,
    Membership.Passive,
    Membership.Pending,
    Membership.Resigned,
  ];

  is_edit: boolean = false;
  member: Member = new_member();
  edit_member: Member = new_member();

  edit() {
    this.is_edit = !this.is_edit;
    this.edit_member = { ... this.member};
  }

  max_date(): Date {
    const today = new Date();
    today.setFullYear(today.getFullYear() - 18);
    return today;
  }

  generate_update_request(): UpdateMember | null {
    if(!this.edit_member){
      return null;
    }

    if(!validate_member(this.edit_member)){
      return null;
    }

    let update: UpdateMember = {};

    let changes = false;

    if(this.member.email !== this.edit_member.email){
      update.email = this.edit_member.email;
      changes = true;
    }
    if(this.member.phone !== this.edit_member.phone){
      update.phone = this.edit_member.phone;
      changes = true;
    }
    if(this.member.first_name !== this.edit_member.first_name){
      update.first_name = this.edit_member.first_name;
      changes = true;
    }
    if(this.member.last_name !== this.edit_member.last_name){
      update.last_name = this.edit_member.last_name;
      changes = true;
    }
    if(this.member.birthday !== this.edit_member.birthday){
      update.birthday = this.edit_member.birthday;
      changes = true;
    }
    if(this.member.postalcode !== this.edit_member.postalcode){
      update.postalcode = this.edit_member.postalcode;
      changes = true;
    }
    if(this.member.city !== this.edit_member.city){
      update.city = this.edit_member.city;
      changes = true;
    }
    if(this.member.street !== this.edit_member.street){
      update.street = this.edit_member.street;
      changes = true;
    }
    if(this.member.house_number !== this.edit_member.house_number){
      update.house_number = this.edit_member.house_number;
      changes = true;
    }
    if(this.member.membership_state !== this.edit_member.membership_state){
      update.membership_state = this.edit_member.membership_state;
      changes = true;
    }

    if(this.member.member_id !== this.edit_member.member_id){
      update.member_id = this.edit_member.member_id;
      changes = true;
    }

    if(!changes){
      return null;
    }

    return update;
  }

  set_member(member: Member) {
    this.member = member;
    this.edit_member = member;
  }

  submit_member() {
    if (!validate_member(this.edit_member)) {
      return;
    }

    if(this.member.id.length < 1){
      const member: NewMember = member_to_new(this.edit_member);
      this.backend_service.create_member(member).subscribe((member: Member) => {
        this.set_member(member);
        this.is_edit = false;
      });
      
      return;
    } else {
      const update = this.generate_update_request();
      if(update){
        this.backend_service.update_member(this.member.id,update).subscribe((member: Member) => {
          console.table(member);
          this.set_member(member);
          this.is_edit = false;
        });
      }
    }

  }
}
