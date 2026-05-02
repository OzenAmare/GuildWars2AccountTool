using GuildWars2Object.Models;
using Microsoft.AspNetCore.Mvc.Formatters;

namespace Items
{
    public class Item : IGuildWars2Object
    {
        private string _item_id { get; set; }

        public string item_id => _item_id;

        private string _item_name { get; set; }

        public string item_name => _item_name;
        
        public async Task GetSingleObjectData(int id)
        {
            
        }
    }
    
}