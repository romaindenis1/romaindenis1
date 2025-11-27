#define MAX_LOGIN_LEN 50
#define MAX_URL_LEN 256

typedef struct {
    char login[MAX_LOGIN_LEN];
    long long id; 
    char avatar_url[MAX_URL_LEN];
    char html_url[MAX_URL_LEN];
} github_user_data;

