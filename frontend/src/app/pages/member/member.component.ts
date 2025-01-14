import { Component } from "@angular/core";
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

@Component({
  selector: "app-member",
  templateUrl: "./member.component.html",
  styleUrls: ["./member.component.scss"],
})
export class MemberComponent {
  constructor(private backend_service: BackendService) {}

  is_new: boolean = true;
  member: Member = new_member();
  create_member() {
    if (!validate_member(this.member)) {
      return;
    }
    const member: NewMember = member_to_new(this.member);
    this.backend_service.create_member(member).subscribe((member: Member) => {
      console.log("create_member response", member);
    });
  }
}
