create table students (
   id integer primary key autoincrement not null,
   name text not null,
   pw text);  /*未使用 */

create table mentors (
   id integer primary key autoincrement not null,
   name text not null);

create table courses (
   id integer primary key autoincrement not null,
   name text not null,
   tutor text); /* 未使用 */

create table histories (
   id integer primary key autoincrement not null,
   student_id integer not null,
   course_id integer not null,
   date  text not null,
   score  integer default 0 not null,
   foreign key (student_id) references students(id),
   foreign key (course_id) references courses(id) );

create table chapters (
   id integer primary key autoincrement not null,
   course_id integer not null,
   name text not null,
   content text not null,
   foreign key (course_id) references courses(id) );

create table exams (
   id integer primary key autoincrement not null,
   course_id integer not null,
   name text not null,
   foreign key (course_id) references courses(id) );

create table questions (
   id integer primary key autoincrement not null,
   exam_id integer not null,
   content text not null,
   foreign key (exam_id) references exams(id) );

create table question_histories (
   id integer primary key autoincrement not null,
   question_id integer not null,
   student_id integer not null,
   correct boolean not null,
   foreign key (question_id) references questions(id)
   foreign key (student_id) references students(id)
   UNIQUE (question_id, student_id)
);

create table exam_histories (
   id integer primary key autoincrement not null,
   exam_id integer not null,
   student_id integer not null,
   start_datetime text not null,
   end_datetime text,
   score integer,
   foreign key (exam_id) references exams(id)
   foreign key (student_id) references students(id)
   UNIQUE (exam_id, student_id)
);

insert into students values(1, 'Yamada', null);
insert into mentors values(1, 'Tanaka');
insert into courses values(1, 'rust programing', null);
insert into courses values(2, 'function programing', null);
insert into chapters values(1, 1, '基本データ型', 'content');
insert into chapters values(2, 1, '構造体と列挙', 'content');
insert into histories values(1, 1, 1, '2023-03-01', 50);
insert into exams values(1, 1, 'データ型試験');
insert into questions values(1, 1, '4要素の配列を作成し、その中に0~9までのランダムな値を格納しなさい');
