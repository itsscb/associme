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
      { label: "ID", },
    ]);
  }

  ngOnInit() {
    this.user_id = this.route.snapshot.paramMap.get("id");
    if (this.user_id) {
      this.breadcrumbService.setItems([
        { label: "Member", routerLink: ["/member"] },
        { label: this.user_id, },
      ]);

      
    }
    this.max_year = new Date();
    this.max_year.setFullYear(this.max_year.getFullYear() - 18);
  }

  user_id: string;

  max_year: Date;

  countries: any[];

  filteredCountries: any[];

  selectedCountryAdvanced: any[];

  valSlider = 50;

  valColor = "#424242";

  valRadio: string;

  valCheck: string[] = [];

  valCheck2: boolean;

  valSwitch: boolean;

  cities: SelectItem[];

  selectedList: SelectItem;

  selectedDrop: SelectItem;

  selectedMulti: string[] = [];

  valToggle = false;

  paymentOptions: any[];

  valSelect1: string;

  valSelect2: string;

  valueKnob = 20;

  membership_options = [
    Membership.None,
    Membership.Active,
    Membership.Passive,
    Membership.Pending,
    Membership.Resigned,
  ];

  is_new: boolean = true;
  member: Member = new_member();

  max_date(): Date {
    const today = new Date();
    today.setFullYear(today.getFullYear() - 18);
    return today;
  }

  create_member() {
    console.log(JSON.stringify(this.member));
    if (!validate_member(this.member)) {
      return;
    }
    navigator.clipboard.writeText(JSON.stringify(this.member));
    // const member: NewMember = member_to_new(this.member);
    // this.backend_service.create_member(member).subscribe((member: Member) => {
    //   console.log("create_member response", member);
    // });
  }
}
