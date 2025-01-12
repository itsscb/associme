export interface Account {
    id: string,
    email: string,
    role: string,
    created_at: Date,
    created_by: string,
    changed_at: Date,
    changed_by: string,
    email_verified_at?: Date,
    verification_sent?: Date,
}