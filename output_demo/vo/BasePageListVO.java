package com.union.api.vo;

import lombok.Data;

import java.util.List;

@Data
public class BasePageListVO<T> {

    private List<T> list;
    private Integer count;

    public BasePageListVO() { }

    public BasePageListVO(List<T> list, Integer count) { 
        this.list = list;
        this.count = count;
    }
}
