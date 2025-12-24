package com.pxc.school.controllers;

import com.pxc.school.dtos.SchoolDto;
import com.pxc.school.dtos.SchoolResponse;
import com.pxc.school.models.School;
import com.pxc.school.services.SchoolService;
import lombok.AllArgsConstructor;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.Map;

@AllArgsConstructor
@RestController
@RequestMapping("/schools")
public class SchoolController {
    private final SchoolService schoolService;

    @PostMapping
    public ResponseEntity<Void> saveSchool(@RequestBody School school){
        schoolService.saveSchool(school);
        return ResponseEntity.status(HttpStatus.CREATED).build();
    }

    @GetMapping
    public List<School> getAllSchools(){
        return schoolService.getAllSchools();
    }

    @GetMapping("/{schoolId}")
    public ResponseEntity<SchoolDto> getSchool(@PathVariable(name = "schoolId") Integer schoolId){
        return ResponseEntity.ok(schoolService.getSchool(schoolId));
    }

    @GetMapping("/with-students/{schoolId}")
    public ResponseEntity<SchoolResponse> getSchoolWithStudents(@PathVariable(name = "schoolId") Integer schoolId){
        return ResponseEntity.ok(schoolService.getSchoolWithStudents(schoolId));
    }

    @ExceptionHandler(RuntimeException.class)
    public ResponseEntity<Map<String, String>> handleRuntime(Exception e){
        return ResponseEntity.status(HttpStatus.NOT_FOUND).body(Map.of("error", e.getMessage()));
    }
}
