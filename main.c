#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <curl/curl.h>
#include <jansson.h>
#include "main.h"

//Struct for response data
typedef struct {
    char *memory;
    size_t size;
} MemoryStruct;

//cURL write callback
size_t write_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t realsize = size * nmemb;
    MemoryStruct *mem = (MemoryStruct *)userp;

    char *ptr = realloc(mem->memory, mem->size + realsize + 1);
    if(ptr == NULL) {
        fprintf(stderr, "Error: not enough memory (realloc returned NULL)\n");
        return 0;
    }

    mem->memory = ptr;
    memcpy(&(mem->memory[mem->size]), contents, realsize);
    mem->size += realsize;
    mem->memory[mem->size] = 0;

    return realsize;
}

//Dynamic extract
void extract_and_print_keys(json_t *root, const char **keys, size_t count) {
    size_t i;
    printf("--- Extracted User Data ---\n");
    
        for (i = 0; i < count; i++) {
        const char *key = keys[i];
        json_t *value_json = json_object_get(root, key);
        
        printf("[%s]: ", key);

        if (!value_json) {
            printf("Key not found.\n");
            continue;
        }

        if (json_is_string(value_json)) {
            printf("%s\n", json_string_value(value_json));
        } 
        else if (json_is_integer(value_json)) {
            printf("%lld\n", (long long)json_integer_value(value_json));
        } 
        else if (json_is_true(value_json)) {
            printf("true\n");
        } 
        else if (json_is_false(value_json)) {
            printf("false\n");
        }
        else if (json_is_null(value_json)) {
            printf("null\n");
        }
    }
}

int main (void)
{
    CURL *curl;
    struct curl_slist *headers = NULL;
    MemoryStruct chunk = {NULL, 0}; 

    const char *keys_to_extract[] = {
        "login",  
        "location", 
        "followers", 
        "following", 
        "bio", 
        "public_repos",
        "total_private_repos",
        "hireable"
    };
    size_t num_keys = sizeof(keys_to_extract) / sizeof(keys_to_extract[0]);


    curl_global_init(CURL_GLOBAL_DEFAULT);
    curl = curl_easy_init();

    if(curl) {
        // cURL setup
        headers = curl_slist_append(headers, "Authorization: Bearer *token*");
        headers = curl_slist_append(headers, "User-Agent: curl/8.0 C-App");
        headers = curl_slist_append(headers, "Accept: application/vnd.github+json");
        headers = curl_slist_append(headers, "X-GitHub-Api-Version: 2022-11-28");

        curl_easy_setopt(curl, CURLOPT_URL, "https://api.github.com/users/romaindenis1");
        curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback);
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, (void *)&chunk);


        //exec cURL
        CURLcode res = curl_easy_perform(curl);
        if(res != CURLE_OK) {
            fprintf(stderr, "curl_easy_perform() failed: %s\n", curl_easy_strerror(res));
            curl_easy_cleanup(curl);
            curl_slist_free_all(headers);
            if (chunk.memory) free(chunk.memory);
            return 1;
        }
        curl_easy_cleanup(curl);
        curl_slist_free_all(headers);

        //jasson time
        if (chunk.memory == NULL) {
            fprintf(stderr, "Error: No data received from API.\n");
            return 1;
        }

        json_error_t error;
        json_t *root = json_loads(chunk.memory, 0, &error);
        
        free(chunk.memory); 
        
        if (!root) {
            fprintf(stderr, "Error parsing JSON (line %d, col %d): %s\n", error.line, error.column, error.text);
            return 1;
        }
        
        if (!json_is_object(root)) {
            fprintf(stderr, "Error: Expected a single JSON object.\n");
            json_decref(root);
            return 1;
        }

        extract_and_print_keys(root, keys_to_extract, num_keys);

        //cleanup jansson
        json_decref(root);
    }
    
    curl_global_cleanup();
    return 0;
}
