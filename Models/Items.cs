using GuildWars2Object.Models;
using Microsoft.AspNetCore.Mvc.Formatters;
using System.Net.Http;
using System.Threading.Tasks;
using System.Text.Json;

namespace Items
{
        public record ItemResponse(
        string Name,
        string Description,
        string Type,
        int Level,
        string Rarity,
        int VendorValue,
        int DefaultSkin,
        List<string> GameTypes,
        List<string> Flags,
        List<object> Restrictions,
        int Id,
        string ChatLink,
        string Icon,
        ItemDetails Details
    );

    public record ItemDetails(
        string Type,
        string DamageType,
        int MinPower,
        int MaxPower,
        int Defense,
        List<InfusionSlot> InfusionSlots,
        double AttributeAdjustment,
        int SuffixItemId,
        List<int> StatChoices,
        string SecondarySuffixItemId
    );

    public record InfusionSlot(
        List<string> Flags
    );
    
    
}