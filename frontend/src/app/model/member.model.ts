import { em } from "@fullcalendar/core/internal-common";

export interface Member {
    id: string,
    email: string,
    phone: string,
    first_name: string,
    last_name: string,
    member_id?: string,
    birthday: Date,
    postalcode: string,
    city: string,
    street: string,
    house_number: string,
    membership_state: Membership,
    resignation_date?: Date,
    resignation_reason?: string,
    created_at: Date,
    created_by: string,
    changed_at: Date,
    changed_by: string,
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
    Active = 'active',
    Passive = 'passive',
    Pending = 'pending',
    Resigned = 'resigned',
    None = 'none',
}