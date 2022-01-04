package com.union.api.service;

import com.union.api.domain.AdUserEntry;
import com.union.api.mapper.AdUserMapper;
import com.union.api.dto.filter.AdUserFilter;
import com.union.api.vo.BasePageListVO;
import com.union.api.vo.AdUserVO;
import org.springframework.stereotype.Service;
import javax.annotation.Resource;
import java.util.ArrayList;
import java.util.List;
import lombok.Data;

@Service
public class AdUserService {

    @Resource
    private AdUserMapper adUserMapper;

    public BasePageListVO getPageList(Integer pageNum, Integer pageSize) { 
        List<AdUserVO> voList = new ArrayList<>();
        Integer limit = pageSize; // TODO construct limit 
        Integer offset = pageNum * pageSize; // TODO construct offset 
        AdUserFilter filter = new AdUserFilter(); // TODO construct filter
        List<AdUserEntry> adUserList = adUserMapper.getPageList(filter, limit, offset);
        Integer count = adUserMapper.getCount(filter);
        for (AdUserEntry adUser: adUserList) {
            voList.add(new AdUserVO(adUser));
        }
        return new BasePageListVO<>(voList, count);
    }

    public AdUserVO getDetail(Long id) { 
        AdUserEntry adUser = adUserMapper.getById(id);
        return new AdUserVO(adUser);
    }

    public void add(String account,
                    String password,
                    String name,
                    String avatar,
                    Integer status,
                    Integer isAdmin) {
        AdUserEntry adUser = new AdUserEntry();
        adUser.setAccount(account);
        adUser.setPassword(password);
        adUser.setName(name);
        adUser.setAvatar(avatar);
        adUser.setStatus(status);
        adUser.setIsAdmin(isAdmin);
        adUserMapper.add(adUser);
    }

    public void update(Long id,
                       String account,
                       String password,
                       String name,
                       String avatar,
                       Integer status,
                       Integer isAdmin) {
        AdUserEntry adUser = new AdUserEntry();
        adUser.setId(id);
        adUser.setAccount(account);
        adUser.setPassword(password);
        adUser.setName(name);
        adUser.setAvatar(avatar);
        adUser.setStatus(status);
        adUser.setIsAdmin(isAdmin);
        adUserMapper.update(adUser);
    }


}
