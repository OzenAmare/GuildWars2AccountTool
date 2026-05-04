using Items;
using Microsoft.Extensions.Options;
using System.Text.Json;
using ApiEndpoints;

namespace ApiServices
{
    public class ItemService
    {

            private readonly HttpClient _http;
            private readonly JsonSerializerOptions _options;
            private  string bananas; 
            
          

            private readonly ArenaNetApi _endpoints;

            public ItemService(
                HttpClient http,
                IOptions<ArenaNetApi> endpoints)
            {
            _http = http;
            _endpoints = endpoints.Value;
            // Use SnakeCaseLower to automatically map "vendor_value" to "VendorValue"
            _options = new JsonSerializerOptions
            {
                PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
                PropertyNameCaseInsensitive = true
            };
                    
            }

            public async Task<ItemResponse?> GetItemAsync(int id)
            {
                bananas = "test variable for trying jetbrains rider";
                return await _http.GetFromJsonAsync<ItemResponse>($"{_endpoints.Items}{id}", _options);

            }
    }
    
}