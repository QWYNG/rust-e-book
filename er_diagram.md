erDiagram

chapters {
  INTEGER id
  INTEGER course_id
  TEXT name
  TEXT content
}

courses {
  INTEGER id
  TEXT name
  TEXT tutor
}

exam_histories {
  INTEGER id
  INTEGER exam_id
  INTEGER student_id
  TEXT start_datetime
  TEXT end_datetime
  INTEGER score
}

exams {
  INTEGER id
  INTEGER course_id
  TEXT name
}

histories {
  INTEGER id
  INTEGER student_id
  INTEGER course_id
  TEXT date
  INTEGER score
}

mentors {
  INTEGER id
  TEXT name
}

question_histories {
  INTEGER id
  INTEGER question_id
  INTEGER student_id
  boolean correct
}

questions {
  INTEGER id
  INTEGER exam_id
  TEXT content
}

students {
  INTEGER id
  TEXT name
  TEXT pw
}

courses ||--o{ chapters : owns

students ||--o{ exam_histories : owns

exams ||--o{ exam_histories : owns

courses ||--o{ exams : owns

courses ||--o{ histories : owns

students ||--o{ histories : owns

students ||--o{ question_histories : owns

questions ||--o{ question_histories : owns

exams ||--o{ questions : owns