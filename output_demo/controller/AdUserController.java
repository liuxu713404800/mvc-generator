package com.union.api.controller;

import com.union.api.service.AdUserService;
import com.union.api.vo.BasePageListVO;
import com.union.api.vo.AdUserVO;
import org.springframework.web.bind.annotation.*;
import javax.annotation.Resource;

@RestController
@RequestMapping("")
public class AdUserController {

    @Resource
    private AdUserService adUserService;

    @GetMapping("getPageList")
    public void getPageList(@RequestParam(value = "page_num") Integer pageNum,
                            @RequestParam(value = "page_size") Integer pageSize) { 
        BasePageListVO res = adUserService.getPageList(pageNum, pageSize);
    }

    @GetMapping("getDetail")
    public void getDetail(@RequestParam(value = "id")Long id) { 
        AdUserVO  res = adUserService.getById(id);
    }

    @PostMapping("add")
    public void add(@RequestParam(value = "account") String account,
                    @RequestParam(value = "password") String password,
                    @RequestParam(value = "name") String name,
                    @RequestParam(value = "avatar") String avatar,
                    @RequestParam(value = "status") Integer status,
                    @RequestParam(value = "is_admin") Integer isAdmin) {
        adUserService.add(account, password, name, avatar, status, isAdmin);
    }

    @PostMapping("update")
    public void update(@RequestParam(value = "id") Long id,
                       @RequestParam(value = "account") String account,
                       @RequestParam(value = "password") String password,
                       @RequestParam(value = "name") String name,
                       @RequestParam(value = "avatar") String avatar,
                       @RequestParam(value = "status") Integer status,
                       @RequestParam(value = "is_admin") Integer isAdmin) {
        adUserService.update(id, account, password, name, avatar, status, isAdmin);
    }


}
