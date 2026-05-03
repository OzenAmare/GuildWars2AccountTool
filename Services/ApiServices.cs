using Items;
using System.Text.Json;

namespace ApiServices
{
    public class ItemService
    {
            private readonly HttpClient _http;
            private readonly JsonSerializerOptions _options;

            public ItemService(HttpClient http)
            {
                _http = http;
                // Use SnakeCaseLower to automatically map "vendor_value" to "VendorValue"
                _options = new JsonSerializerOptions
                {
                    PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
                    PropertyNameCaseInsensitive = true
                };
            }

            public async Task<ItemResponse?> GetItemAsync(int id)
            {
                // One-liner to fetch and deserialize into your records
                return await _http.GetFromJsonAsync<ItemResponse>($"https://api.guildwars2.com/v2/items/30704", _options);
            }
    }
    
}