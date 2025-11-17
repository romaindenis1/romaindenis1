#include <stdio.h>
#include <stdlib.h>
#include <curl/curl.h>

int main (void)
{
    CURL *curl;
    struct curl_slist *headers = NULL;

    curl = curl_easy_init();
    if(curl) {
        headers = curl_slist_append(headers, "Authorization: Bearer (i dont have secret thing yet so im removing it here for commit)");
        headers = curl_slist_append(headers, "User-Agent: curl/8.0");
        headers = curl_slist_append(headers, "Accept: application/vnd.github+json");
        headers = curl_slist_append(headers, "X-GitHub-Api-Version: 2022-11-28");

        curl_easy_setopt(curl, CURLOPT_URL, "https://api.github.com/users/romaindenis1");
        curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);

        CURLcode res = curl_easy_perform(curl);
        if(res != CURLE_OK) {
            fprintf(stderr, "curl error: %s\n", curl_easy_strerror(res));
        }

        curl_easy_cleanup(curl);
        curl_slist_free_all(headers);
    }

    return 0;
}
