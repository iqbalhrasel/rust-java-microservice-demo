package com.pxc.school.dtos;

import lombok.AllArgsConstructor;
import lombok.Data;

@AllArgsConstructor
@Data
public class SchoolDto {
    private Integer id;
    private String name;
    private String email;
}
