using GuildWars2Object.Models;
using Microsoft.AspNetCore.Mvc.Formatters;

namespace Items
{
    public class Item : IGuildWars2Object
    {
        public async Task GetObjectData(int id)
        {

        }

        //to finish this go to this https://api.guildwars2.com/v2/items/30704
        private string _item_id;

        public string item_id => _item_id;

        private string _item_name { get; set; }

        public string item_name => _item_name;

        private string _item_type { get; set; }

        public string item_type => _item_type;

        private int _item_level { get; set; }

        public int item_level => _item_level;

        private string _item_rarity { get; set; }

        public string item_rarity => _item_rarity;

        private int _vendor_value { get; set; }

        public int vendor_value => _vendor_value;

        private int _default_skin { get; set; }

        public int default_skin => _default_skin;

        private Dictionary<int, string> _game_types { get; set; }

        public Dictionary<int, string> game_types => _game_types;

        private Dictionary<int, string> _flags { get; set; }

        public Dictionary<int, string> flags => _flags;

        private List<string> _restrictions { get; set; }

        public List<string> restrictions => _restrictions;

        private string _chat_link { get; set; }

        public string chat_link => _chat_link;


        //this is literally a link to an icon. Maybe we could do something better?
        private string _icon { get; set; }

        public string icon => _icon;

        
    
        protected class item_details
        {
            private string _item_type { get; set; }

            public string item_type => _item_type; 

            private string _damage_type { get; set; }

            public string damage_type => _damage_type; 

            private int _min_power { get; set; }

            public int min_power => _min_power;

            private int _max_power { get; set; }

            public int max_power => _max_power; 
        }
    }
    
    
}