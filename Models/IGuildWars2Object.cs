using System.Numerics;
using System.Reflection.Metadata.Ecma335;

namespace GuildWars2Object.Models
{
    public interface IGuildWars2Object
    {
        //we'll need to use this for the different things in GW2 

        public async Task GetSingleObjectData(int id) { }

        
    }
    
}