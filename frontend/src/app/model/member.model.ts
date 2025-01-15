export function new_member(): Member {
  const today = new Date();
  today.setFullYear(today.getFullYear() - 18);
  const member: Member = {
    id: "",
    email: "",
    phone: "",
    first_name: "",
    last_name: "",
    birthday: today,
    postalcode: "",
    city: "",
    street: "",
    house_number: "",
    membership_state: Membership.None,
    created_at: new Date(),
    created_by: "",
    changed_at: new Date(),
    changed_by: "",
  };
  return member;
}

export function member_to_new(member: Member): NewMember {
  const new_member: NewMember = {
    email: member.email,
    phone: member.phone,
    first_name: member.first_name,
    last_name: member.last_name,
    birthday: member.birthday,
    postalcode: member.postalcode,
    city: member.city,
    street: member.street,
    house_number: member.house_number,
    membership_state: member.membership_state,
    member_id: parseInt(member.member_id),
  };
  return new_member;
}

export function validate_member(member: Member): boolean {
  if (member.email === "") {
    return false;
  }
  if (member.phone === "") {
    return false;
  }
  if (member.first_name === "") {
    return false;
  }
  if (member.last_name === "") {
    return false;
  }
  if (member.birthday === null) {
    return false;
  }
  if (member.postalcode === "") {
    return false;
  }
  if (member.city === "") {
    return false;
  }
  if (member.street === "") {
    return false;
  }
  if (member.house_number === "") {
    return false;
  }

  return true;
}

export interface Member {
  id: string;
  email: string;
  phone: string;
  first_name: string;
  last_name: string;
  member_id?: string;
  birthday: Date;
  postalcode: string;
  city: string;
  street: string;
  house_number: string;
  membership_state: Membership;
  resignation_date?: Date;
  resignation_reason?: string;
  created_at: Date;
  created_by: string;
  changed_at: Date;
  changed_by: string;
}

export interface NewMember {
  email: string;
  phone: string;
  first_name: string;
  last_name: string;
  birthday: Date;
  postalcode: string;
  city: string;
  street: string;
  house_number: string;
  membership_state: Membership;
  member_id?: number;
}

export enum Membership {
  Active = "active",
  Passive = "passive",
  Pending = "pending",
  Resigned = "resigned",
  None = "none",
}
