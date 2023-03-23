create table students (
   id integer primary key autoincrement not null,
   name text not null,
   pw text);  /*未使用 */

create table courses (
   id integer primary key autoincrement not null,
   name text not null,
   tutor text); /* 未使用 */

create table historys (
   id integer primary key autoincrement not null,
   student_id integer not null,
   course_id integer not null,
   date  text not null,
   score  integer default 0 not null,
   foreign key (student_id) references student(id),
   foreign key (course_id) references course(id) );

insert into students values(1, 'Yamada', null);
insert into courses values(1, 'network security', null);