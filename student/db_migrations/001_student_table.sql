create table students(
    id int auto_increment,
    first_name varchar(255) not null,
    last_name varchar(255),
    email varchar(255) not null,
    school_id int not null,
    primary key(id)
);
