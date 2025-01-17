import { Component, OnInit } from "@angular/core";
import { CommonModule } from "@angular/common";

import {
  Member,
  member_to_new,
  Membership,
  new_member,
  NewMember,
  validate_member,
} from "src/app/model/member.model";
import { BackendService } from "src/app/service/backend.service";
import { SelectItem } from "primeng/api";
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
    private breadcrumbService: BreadcrumbService,
    private route: ActivatedRoute,
  ) {
    this.breadcrumbService.setItems([
      { label: "Member", routerLink: ["/member"] },
      { label: "New", },
    ]);
  }

  ngOnInit() {
    this.user_id = this.route.snapshot.paramMap.get("id");
    if (this.user_id === 'new') {
      this.breadcrumbService.setItems([
        { label: "Member", routerLink: ["/member"] },
        { label: "New", },
      ]);
      this.is_new = true;
      this.is_edit = true;
    }
    else if(this.user_id) {
      this.backend_service.get_member(this.user_id).subscribe((member) => {
        console.table(member);
        this.member = member;
        this.breadcrumbService.setItems([
          { label: "Member", routerLink: ["/member"] },
          { label: `${member.last_name}, ${member.first_name}${member.member_id ? ` [${member.member_id}]` : ''}`, },
        ]);
        this.is_new = false;
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

  is_new: boolean = true;
  is_edit: boolean = false;
  member: Member = new_member();

  max_date(): Date {
    const today = new Date();
    today.setFullYear(today.getFullYear() - 18);
    return today;
  }

  submit_member() {
    if (!validate_member(this.member)) {
      return;
    }

    const member: NewMember = member_to_new(this.member);
    this.backend_service.create_member(member).subscribe((member: Member) => {
      console.log("create_member response", member);
    });
  }
}
