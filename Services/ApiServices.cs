using Items;

namespace ApiServices
{
    public class ItemService
    {
        private readonly HttpClient _client;
        public ItemService(HttpClient client) => _client = client; 

        public async Task<Item> GetItemAsync(int id)
        {
            return await _client.GetFromJsonAsync<Item>("https://api.guildwars2.com/v2/items/30704");
        }
    }
    
}