mod c_library {
    #[allow(non_camel_case_types)]
    type esp_err_t = cty::c_int;

    const ESP_ERR_WIFI_BASE : usize = 0x3000;
    const ESP_ERR_ESPNOW_BASE       : usize = ESP_ERR_WIFI_BASE + 100;
    const ESP_ERR_ESPNOW_NOT_INIT   : usize = ESP_ERR_ESPNOW_BASE + 1;
    const ESP_ERR_ESPNOW_ARG        : usize = ESP_ERR_ESPNOW_BASE + 2;
    const ESP_ERR_ESPNOW_NO_MEM     : usize = ESP_ERR_ESPNOW_BASE + 3;
    const ESP_ERR_ESPNOW_FULL       : usize = ESP_ERR_ESPNOW_BASE + 4;
    const ESP_ERR_ESPNOW_NOT_FOUND  : usize = ESP_ERR_ESPNOW_BASE + 5;
    const ESP_ERR_ESPNOW_INTERNAL   : usize = ESP_ERR_ESPNOW_BASE + 6;
    const ESP_ERR_ESPNOW_EXIST      : usize = ESP_ERR_ESPNOW_BASE + 7;
    const ESP_ERR_ESPNOW_IF         : usize = ESP_ERR_ESPNOW_BASE + 8;

    const ESP_NOW_ETH_ALEN             : usize = 6;
    const ESP_NOW_KEY_LEN              : usize = 16;

    const ESP_NOW_MAX_TOTAL_PEER_NUM   : usize = 20;
    const ESP_NOW_MAX_ENCRYPT_PEER_NUM : usize = 6;

    const ESP_NOW_MAX_DATA_LEN         : usize = 250;

    #[repr(C)]
    pub enum esp_now_send_status_t {
        ESP_NOW_SEND_SUCCESS = 0,
        ESP_NOW_SEND_FAIL,
    }

    #[repr(C)]
    pub enum wifi_interface_t {
        WIFI_IF_STA = 0,
        WIFI_IF_AP,
    }

    #[repr(C)]
    pub struct esp_now_peer_info_t {
        peer_addr: [cty::uint8_t; ESP_NOW_ETH_ALEN],
        lmk: [cty::uint8_t; ESP_NOW_KEY_LEN],
        channel: cty::uint8_t,
                                                
                                                
        ifidx: wifi_interface_t,
        encrypt: bool,
        priv_: cty::c_void,
    }

    #[repr(C)]
    pub struct esp_now_peer_num_t {
        total_num: cty::c_int,
        encrypt_num: cty::c_int,
    }

    #[allow(non_camel_case_types)]
    type esp_now_recv_cb_t = extern "C" fn (mac_addr: *const cty::uint8_t, data: *const cty::uint8_t, data_len: cty::c_int);
    #[allow(non_camel_case_types)]
    type esp_now_send_cb_t = extern "C" fn (mac_addr: *const cty::uint8_t, status: esp_now_send_status_t);


    #[link(kind="static", name="espnow")]
    extern {
        /*
        esp_err_t esp_now_init(void);
        esp_err_t esp_now_deinit(void);
        esp_err_t esp_now_get_version(uint32_t *version);
        esp_err_t esp_now_register_recv_cb(esp_now_recv_cb_t cb);
        esp_err_t esp_now_unregister_recv_cb(void);
        esp_err_t esp_now_register_send_cb(esp_now_send_cb_t cb);
        esp_err_t esp_now_unregister_send_cb(void);
        esp_err_t esp_now_add_peer(const esp_now_peer_info_t *peer);
        esp_err_t esp_now_del_peer(const uint8_t *peer_addr);
        esp_err_t esp_now_mod_peer(const esp_now_peer_info_t *peer);
        esp_err_t esp_now_get_peer(const uint8_t *peer_addr, esp_now_peer_info_t *peer);
        esp_err_t esp_now_fetch_peer(bool from_head, esp_now_peer_info_t *peer);
        bool esp_now_is_peer_exist(const uint8_t *peer_addr);
        esp_err_t esp_now_get_peer_num(esp_now_peer_num_t *num);
        esp_err_t esp_now_set_pmk(const uint8_t *pmk);
        esp_err_t esp_now_set_wake_window(uint16_t window);
        */
        fn esp_now_init() -> esp_err_t;
        fn esp_now_deinit() -> esp_err_t;
        fn esp_now_get_version(version: *mut cty::uint32_t) -> esp_err_t;

        fn esp_now_register_recv_cb(cb: esp_now_recv_cb_t) -> esp_err_t;
        fn esp_now_unregister_recv_cb() -> esp_err_t;
        fn esp_now_register_send_cb(cb: esp_now_send_cb_t) -> esp_err_t;
        fn esp_now_unregister_send_cb() -> esp_err_t;

        fn esp_now_add_peer(peer: *const esp_now_peer_info_t) -> esp_err_t;
        fn esp_now_del_peer(peer_addr: *const cty::uint8_t) -> esp_err_t;
        fn esp_now_mod_peer(peer: *const esp_now_peer_info_t) -> esp_err_t;
        fn esp_now_get_peer(peer_addr: *const cty::uint8_t, peer: *mut esp_now_peer_info_t) -> esp_err_t;
        fn esp_now_fetch_peer(from_head: bool, peer: *mut esp_now_peer_info_t) -> esp_err_t;
        fn esp_now_is_peer_exist(peer_addr: *const cty::uint8_t) -> bool;
        fn esp_now_get_peer_num(num: *mut esp_now_peer_num_t) -> esp_err_t;
        fn esp_now_set_pmk(pmk: *const cty::uint8_t) -> esp_err_t;
        fn esp_now_set_wake_window(window: cty::uint16_t) -> esp_err_t;
    }
}
