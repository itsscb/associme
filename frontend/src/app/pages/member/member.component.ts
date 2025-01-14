import { Component } from "@angular/core";
import { CommonModule } from "@angular/common";
import { Member, Membership, NewMember } from "src/app/model/member.model";
import { BackendService } from "src/app/service/backend.service";

@Component({
  selector: "app-member",
  templateUrl: "./member.component.html",
  styleUrls: ["./member.component.scss"],
})
export class MemberComponent {
  constructor(private backend_service: BackendService) {}

  is_new: boolean = true;

  create_member() {
    const member: NewMember = {
      email: "test@test.com",
      phone: "123456789",
      first_name: "Max",
      last_name: "Mustermann",
      member_id: 8,
      birthday: new Date(),
      postalcode: "12345",
      city: "Musterstadt",
      street: "Musterstraße",
      house_number: "1",
      membership_state: Membership.Passive,
    };
    this.backend_service.create_member(member).subscribe((member: Member) => {
      console.log("create_member response", member);
    });
  }
}
