export type MultiLangString = {
  th: string;
  "en-US"?: string;
};

export type Class = {
  id: number;
  number: number;
  advisors: Teacher[];
  contacts: Contact[];
  students: Student[];
  year: number;
  // subjects: SubjectListItem[];
};

export type ClassWNumber = Pick<Class, "id" | "number">;

export type ShirtSize =
  | "XS"
  | "S"
  | "M"
  | "L"
  | "XL"
  | "2XL"
  | "3XL"
  | "4XL"
  | "5XL"
  | "6XL";

export type Role = "student" | "teacher";

export type Person = {
  id: number;
  prefix: MultiLangString;
  role: Role;
  first_name: MultiLangString;
  middle_name?: MultiLangString;
  last_name: MultiLangString;
  sex: "male" | "female";
  blood_group?: "O+" | "O-" | "A+" | "A-" | "B+" | "B-" | "AB+" | "AB-";
  profile?: string;
  citizen_id?: string;
  passport_id?: string;
  birthdate: string;
  shirt_size?: ShirtSize;
  pants_size?: string;
  contacts: Contact[];
  is_admin?: boolean;
};

export type Student = Person & {
  role: "student";
  student_id: string;
  class?: ClassWNumber;
  classNo: number;
};

export type Teacher = Person & {
  role: "teacher";
  teacher_id: string;
  class_advisor_at?: ClassWNumber;
  // subject_group: SubjectGroup;
  // subjects_in_charge?: SubjectWNameAndCode[];
};

export type Contact = {
  id: number;
  name?: MultiLangString;
  type: ContactVia;
  includes_parents?: boolean;
  includes_students?: boolean;
  includes_teachers?: boolean;
  value: string;
};

export type ContactVia =
  | "Phone"
  | "Email"
  | "Facebook"
  | "Line"
  | "Instagram"
  | "Website"
  | "Discord"
  | "Other";

interface ClubJoinRequest {
  id: number;
  club: Club;
  student: Student;
  status: "pending" | "accepted" | "rejected";
}

export type Organization = {
  id: string;
  name: MultiLangString;
  description: MultiLangString;
  main_room?: string;
  rooms?: string[];
};

export type Club = Organization & {
  members: Student[];
  staffs: Student[];
  advisors: Teacher[];
  contacts: Contact[];
  logo?: string;
  background_color?: string;
  accent_color?: string;
};
