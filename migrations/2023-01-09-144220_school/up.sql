-- Your SQL goes here
CREATE TABLE teachers (
  teacher_id INTEGER PRIMARY KEY NOT NULL,
  teacher_name VARCHAR NOT NULL,
  subject_name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  FOREIGN KEY (subject_name) REFERENCES Subs(subject_name),
  FOREIGN KEY (teacher_id) REFERENCES Subs(teacher_id)
);

CREATE TABLE students (
  student_id INTEGER PRIMARY KEY NOT NULL,
  student_name VARCHAR NOT NULL,
  class_id INTEGER NOT NULL,
  contact_info VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  FOREIGN KEY (class_id) REFERENCES Subs(class_id),
  FOREIGN KEY (student_id) REFERENCES Grades(student_id)
);

CREATE TABLE subs (
  class_id INTEGER NOT NULL,
  subject_name VARCHAR PRIMARY KEY NOT NULL,
  teacher_id INTEGER NOT NULL,
  FOREIGN KEY (teacher_id) REFERENCES Teachers(teacher_id),
  FOREIGN KEY (subject_name) REFERENCES Grades(subject_name)
);

CREATE TABLE grades (
  grade_id INTEGER PRIMARY KEY NOT NULL,
  student_id INTEGER NOT NULL,
  subject_name VARCHAR NOT NULL,
  assignment_score INTEGER NOT NULL,
  test_score INTEGER NOT NULL,
  FOREIGN KEY (student_id) REFERENCES Students(student_id),
  FOREIGN KEY (subject_name) REFERENCES Classes(subject_name)
);
