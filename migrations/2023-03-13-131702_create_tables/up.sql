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

create table chapters (
   id integer primary key autoincrement not null,
   course_id integer not null,
   name text not null,
   content text not null,
   foreign key (course_id) references course(id) );

insert into students values(1, 'Yamada', null);
insert into courses values(1, 'rust programing', null);
insert into courses values(2, 'function programing', null);
insert into chapters values(1, 1, '基本データ型', 'content');
insert into chapters values(2, 1, '構造体と列挙', 'content');
insert into historys values(1, 1, 1, '2023-03-01', 50);
